# Get all Todos

Get all saved Todo tasks

**URL** : `/todo`

**Method** : `GET`

## Request
No body must be send.

## Success Response

**Code** : `200 OK`

#### Fields
* `id`
    * **Type**: i64
    * **Note**: Id of the created Todo.
* `todo_message`
    * **Type**: String
    * **Note**: The message of the Todo task.
* `completed`
    * **Type**: Boolean
    * **Note**: Indication if the Todo is completed or not.

#### Example


```json
[
  {
    "id": 1,
    "todo_message": "Buy some milk",
    "completed": false
  },
  {
    "id": 2,
    "todo_message": "Finish SE-19 Assessment",
    "completed": true
  }
]
```
