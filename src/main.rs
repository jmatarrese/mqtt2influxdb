use std::error::Error;
use std::str::FromStr;
use rumqttc::{self, AsyncClient, Event, Incoming, MqttOptions, QoS};
use influx_db_client::{Client as Dbclient, Point, Value, Precision, point};
use url::Url;
use serde::{Serialize, Deserialize};
use confy;

#[derive(Serialize, Deserialize)]
struct ConfyConfig {
    db_url: String,
    db_name: String,
    db_user: String,
    db_password: String,
    mqttserverurl: String,
    mqttserverport: u16,
    topics: Vec<String>
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for ConfyConfig {
    fn default() -> Self { 
                        Self { 
                            db_url: "http://192.168.0.200:8086".into(), 
                            db_name: "mqtt2influxdb".into(), 
                            db_user: "root".into(), 
                            db_password: "root".into(),
                            mqttserverurl: "192.168.0.200".into(),
                            mqttserverport: 1883,
                            topics: vec![] 
                            } 
                        }
}

#[tokio::main(worker_threads = 1)]
async fn main() -> Result<(), Box<dyn Error>> {

    let cfg: ConfyConfig = confy::load_path("./mqtt2influxdb.toml")?;

    let dburl = Url::parse(cfg.db_url.as_str()).unwrap();
    let dbclient = Dbclient::new(dburl,cfg.db_name).set_authentication(cfg.db_user, cfg.db_password);

    let mqttoptions = MqttOptions::new("mqtt2influx-1", cfg.mqttserverurl, cfg.mqttserverport);
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    // client.subscribe("sensor1/temp", QoS::AtMostOnce).await.unwrap();
    // client.subscribe("sensor1/humid", QoS::AtMostOnce).await.unwrap();
    // client.subscribe("sensor2/temp", QoS::AtMostOnce).await.unwrap();
    // client.subscribe("sensor2/humid", QoS::AtMostOnce).await.unwrap();
    // client.subscribe("sensor2/pressure", QoS::AtMostOnce).await.unwrap();

    for topic in cfg.topics
    {
        client.subscribe(topic, QoS::AtMostOnce).await.unwrap();
    }
    loop 
    {
        match eventloop.poll().await
        {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                println!("Topic: {}, Payload: {:?}", p.topic, p.payload);
                let tokens:Vec<&str>= p.topic.split("/").collect();
                let sensor = tokens[0];
                let property = tokens[1];
                let strpayload = String::from_utf8_lossy(&p.payload);
                let floatpayload = f64::from_str(&strpayload).unwrap();
                let point = point!(sensor)
                            .add_field(property, Value::Float(floatpayload));
                dbclient.write_point(point, Some(Precision::Seconds), None).await.unwrap();
            }
            Ok(Event::Incoming(i)) => {
                println!("Incoming = {:?}", i);
            }
            Ok(Event::Outgoing(o)) => {
                println!("Outgoing = {:?}", o)
            }
            Err(e) => {
                println!("Error = {:?}", e);
                continue;
            }
        }
    }
}