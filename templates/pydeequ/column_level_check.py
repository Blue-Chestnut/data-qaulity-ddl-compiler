from pyspark.sql import SparkSession, DataFrame
from pydeequ import deequ_maven_coord, f2j_maven_coord
from pyspark.sql.functions import lit
from pydeequ.checks import Check, CheckLevel, ConstrainableDataTypes
from pydeequ.verification import VerificationSuite, VerificationResult

{% for column_level_check in column_level_checks %}
def column_level_checks_{{column_level_check["column_name"]|lower}}(data_frame: DataFrame, spark_session: SparkSession) -> tuple[str, DataFrame | None]:
    try:
        data_frames = []
        {% for filter in column_level_check["filter_checks"] -%} {%if filter["has_filter"]%}
        data_frame_filtered = data_frame.filter("{{filter["filter"]}}")
        check = Check(spark_session, CheckLevel.Warning,
                      "{{filter["description"]}}")
        check_result = VerificationSuite(spark_session).onData(data_frame_filtered).addCheck(
            check{% for check in filter["checks"] %}
            {{check}}
            {%- endfor %}
        ).run(){%else%}
        check = Check(spark_session, CheckLevel.Warning,
                      "{{filter["description"]}}")
        check_result = VerificationSuite(spark_session).onData(data_frame).addCheck(
            check{% for check in filter["checks"] %}
            {{check}}
            {%- endfor %}
        ).run(){%endif%}

        result_df = VerificationResult.checkResultsAsDataFrame(spark_session, check_result)
        result_df = (result_df.withColumn("check_category", lit("column level"))
                     .withColumn("columns", lit("{{column_level_check["ext_column_name"]|lower}}"))
                     .withColumn("filter", lit("{{filter["filter"]}}")))
        data_frames.append(result_df)
        {% endfor %}
        final_df = None
        for result_df in data_frames:
            if final_df is None:
                final_df = result_df
            else:
                final_df = final_df.union(result_df)
        return 'success', final_df

    except Exception as e:
        return f'failure: {e}', None

{% endfor %}
def check_column_level(data_frame: DataFrame, spark_session: SparkSession) -> tuple[DataFrame | None, list[tuple[str, str]]]:
    {% for column_level_check in column_level_checks -%}
    column_level_checks_{{column_level_check["column_name"]|lower}}_df = column_level_checks_{{column_level_check["column_name"]|lower}}(data_frame, spark_session)
    {% endfor -%}

    checks = {
        {% for column_level_check in column_level_checks -%}
        'column_level_checks_{{column_level_check["column_name"]|lower}}': column_level_checks_{{column_level_check["column_name"]|lower}}_df,
        {% endfor -%}
    }

    combined_result_df = None
    failed_checks = []

    for key, (is_success, data) in checks.items():
        if is_success == 'success':
            if combined_result_df is None:
                combined_result_df = data
            else:
                combined_result_df = combined_result_df.union(data)
        else:
            failed_checks.append((key, is_success))
    combined_result_df.show()
    return combined_result_df, failed_checks


def check_table(data_frame: DataFrame, spark_session: SparkSession) -> tuple[DataFrame | None, list[tuple[str, str]]]:
    return check_column_level(data_frame, spark_session)


if __name__ == '__main__':
    # example usage
    spark = (SparkSession.builder
             .config("spark.jars.packages", deequ_maven_coord)
             .config("spark.jars.excludes", f2j_maven_coord).appName('test').getOrCreate())
    df = spark.read.csv('./data/test.csv', header=True, inferSchema=True)
    check_table(df, spark)

    spark.sparkContext.stop()
    spark.stop()
