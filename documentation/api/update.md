# Update Todo

Update the completed status of an existing Todo task.

**URL** : `/todo/{todo_id}`

**Method** : `PUT`

## Request
#### Fields
* `completed`
    * **Type**: Boolean
    * **Note**: Indication if the Todo is completed or not.
    
#### Example
```json
{
  "completed": true
}
```

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
{
  "id": 1,
  "todo_message": "Buy some milk",
  "completed": true
}
```
