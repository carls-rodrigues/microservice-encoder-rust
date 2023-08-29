# Rust Microserviço Encoder with Hexagonal Architecture

## Environment Setup
To run in development mode, follow these steps:

1. Duplicate the `.env.example` file to `.env`.
2. Run the command `docker-compose up -d`.
3. Access the administration of RabbitMQ and create a fanout exchange. This will serve as a Dead Letter Exchange (DLX) to receive messages that are not successfully processed.
4. Create a Dead Letter Queue and bind it to the recently created Dead Letter Exchange. No `routing_key` is required.
5. In the `.env` file, specify the name of the Dead Letter Exchange using the parameter: `RABBITMQ_DLX`.
6. Create a service account on Google Cloud Platform (GCP) with the permission to write to Google Cloud Storage. Download the JSON file containing the credentials and save it in the root of the project with the name: `bucket-credential.json`.

## Running
To execute the encoder, run the following command inside the container. For example:

```sh
docker exec encoder-new2_app_1 make server
```

Here, `microsservico-enconder_app_1` is the name of the container generated by `docker-compose`.

## Message Sending Format to the Encoder
For a message to be parsed by the encoder system, it must arrive in the following JSON format:

```json
{
  "resource_id": "my-resource-id-can-be-a-uuid-type",
  "file_path": "convite.mp4"
}
```
- `resource_id`: Represents the ID of the video you wish to convert. It is of string type.
- `file_path`: The complete path of the MP4 video within the bucket.

## Message Return Format from the Encoder
Processing Success:
For each successfully processed video, the encoder will send the processing result to an exchange (to be configured in `.env`).

If the processing is successfully completed, the JSON return pattern will be:

```json
{
    "id": "bbbdd123-ad05-4dc8-a74c-d63a0a2423d5",
    "output_bucket_path": "bucket-name",
    "status": "COMPLETED",
    "video":{
        "encoded_video_folder":"b3f2d41e-2c0a-4830-bd65-68227e97764f",
        "resource_id":"aadc5ff9-0b0d-13ab-4a40-a11b2eaa148c",
        "file_path":"video.mp4"
    },
    "Error":"",
    "created_at":"2020-05-27T19:43:34.850479-04:00",
    "updated_at":"2020-05-27T19:43:38.081754-04:00"
}
```
Here, `encoded_video_folder` is the folder containing the converted video.

Processing Error:
If the processing encounters an error, the JSON return pattern will be:

```json
{
    "message": {
        "resource_id": "aadc5ff9-010d-a3ab-4a40-a11b2eaa148c",
        "file_path": "convite.mp4"
    },
    "error":"Motivo do erro"
}
```
Additionally, the encoder will send the original message that encountered processing problems to a dead letter exchange (DLX). Configure the desired DLX in the `.env` file using the parameter: `RABBITMQ_DLX`.