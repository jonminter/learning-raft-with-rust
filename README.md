```
docker compose run -it kafka-broker /opt/kafka/bin/kafka-topics.sh --create --topic test_glommio --partitions 16 --bootstrap-server localhost:9092 --replication-factor 1
```