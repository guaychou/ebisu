# Ebisu [WIP]

### What is this ? 

#### My personal project to getting alert from rise to telegram

#### There are some metrics (prefix --> /api/v1)
- alert 
- alertwithmessage

### Request Body
```yaml

{
    "service" : "malganis", #  min 3 , max 20 character
    "message" : "test-testt" # only for alertwithmessage , min 5 character
}

```

### Response Body

#### Success
```yaml
{
    "ok": true,
    "result": {
        "message_id": 61
    }
}

```

#### Validation Error 
```yaml
{
  "errors": {
      "service": [
          {
              "code": "length",
              "message": null,
              "params": {
                  "max": 20,
                  "min": 3,
                  "value": "ry"
              }
          }
      ]}
}
```


### Example ebisu config

```yaml
telegram:
  token: <YOUR_TELEGRAM_TOKEN>
  chat_id: <YOUR_TELEGRAM_CHAT_ID>
server:
  port: 8080
```


### How to run

#### Manual
```shell
$ ebisu --config.ebisu="<PATH_TO_EBISU_CONFIG>"
```

#### Using prebuilt docker image

```shell
$ docker run -it -p8080:8080 -v "<PATH_TO_EBISU_CONFIG>":/app/config/ebisu.yaml -v -d lordchou/ebisu:v0.2.0 \
  ./ebisu --config.ebisu=./config/ebisu.yaml
```