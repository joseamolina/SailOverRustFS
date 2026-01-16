from pyspark.sql import SparkSession

spark = (SparkSession.builder
         .remote("sc://localhost:50051")
         .getOrCreate())

spark.sql("SHOW DATABASES").show(truncate=False)

data = [("ID001", "Sensor_A", 25.5), ("ID002", "Sensor_B", 22.1)]
columns = ["id", "device", "value"]
df = spark.createDataFrame(data, columns)

path_rustfs = "s3a://testsail/prueba_sensores"

print(f"Escribiendo en: {path_rustfs}")
df.write.mode("overwrite").parquet(path_rustfs)
print(f"Escritura realizada en: {path_rustfs}")
