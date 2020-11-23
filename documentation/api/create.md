# Create Todo

Create a new Todo task.

**URL** : `/todo`

**Method** : `POST`

## Request
#### Fields
* `todo_message`
    * **Type**: String
    * **Note**: The message of the Todo task.
    
#### Example
```json
{
  "todo_message": "Buy some milk"
}
```

## Success Response

**Code** : `201 OK`

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
{
  "id": 1,
  "todo_message": "Buy some milk",
  "completed": false
}
```
