const todoInput = document.querySelector('.todo_input');
const todoButton = document.querySelector('.todo_button');
const todoList = document.querySelector('.todo_list');

todoButton.addEventListener("click", addNewTodo)
todoList.addEventListener("click", deleteCheck)

const baseTodoURL = "http://localhost:8087/todo"

var todos = []

// Load Todos into the DOM
function loadTodos() {
    todos.forEach( todo =>
        addSingleTodo(todo)
    )
}

function addSingleTodo(todo) {
    // Wrapping Div
    const todoDiv = document.createElement('div');
    todoDiv.classList.add('todo_element');
    todoDiv.setAttribute("id", todo.id)
    if (todo.completed) todoDiv.classList.add('completed-item');

    // Green Checkmark Button
    const completedButton = document.createElement('button');
    completedButton.innerHTML = '<i class="material-icons">check_circle_outline</i>';
    completedButton.classList.add('complete_btn');
    if (todo.completed) completedButton.children[0].classList.add("md-completed")
    todoDiv.appendChild(completedButton);

    // Todo Message
    const newTodo = document.createElement('li');
    newTodo.innerText = todo.todo_message;
    newTodo.classList.add('todo_item');
    todoDiv.appendChild(newTodo);

    // Delete Button
    const deleteButton = document.createElement('button');
    deleteButton.innerHTML = '<i class="material-icons">delete</i>';
    deleteButton.classList.add('delete_btn')
    todoDiv.appendChild(deleteButton);

    todoList.appendChild(todoDiv);
}

function deleteCheck(event) {
    const item = event.target;
    // Delete todo
    if (item.classList[0] === "delete_btn") {
        const todo = item.parentElement;
        deleteTodo(event, todo.id)
        todo.remove();
    }
    // Complete todo
    if (item.classList[0] === "todo_item" || item.classList[0] === "complete_btn") {
        const todo = item.parentElement;
        updateTodo(event, todo.id, !todo.classList.contains("completed-item"))

        todo.classList.toggle("completed-item")
        todo.querySelector('.complete_btn').children[0].classList.toggle("md-completed")
    }
}

// ------------------------------------------------------------------
// HTTP Requests
// ------------------------------------------------------------------
function fetchTodosReqListener () {
    console.log(this.responseText);
    todos = JSON.parse(this.responseText);
    loadTodos();
}

// Fetch Todos before page is loading
// GET Todos
function fetchTodos()
{
    console.log("FETCH TODOS");
    var xmlhttp = new XMLHttpRequest();
    xmlhttp.addEventListener("load", fetchTodosReqListener);
    xmlhttp.open("GET", baseTodoURL, false);
    xmlhttp.send();
    var x = 5;
}

function createTodoReqListener(httpResponse) {
    const todo = JSON.parse(httpResponse.responseText);
    addSingleTodo(todo);
}

// POST Todo
function addNewTodo(event) {
    event.preventDefault();
    if (todoInput.value === "") return null

    const params = {
        todo_message: todoInput.value
    }

    var xmlhttp = new XMLHttpRequest();
    //xmlhttp.addEventListener("load", createTodoReqListener);
    xmlhttp.open("POST", baseTodoURL, false);
    xmlhttp.setRequestHeader("Content-Type", "application/json; charset=utf-8");
    xmlhttp.send(JSON.stringify(params));
    if (xmlhttp.status != 201) {
        alert(`Error ${xmlhttp.status}: ${xmlhttp.response}`);
    } else {
        const todo = JSON.parse(xmlhttp.responseText);
        addSingleTodo(todo);
    }
    todoInput.value = "";
}

// DELETE Todo
function deleteTodo(event, id) {
    event.preventDefault();
    var xmlhttp = new XMLHttpRequest();
    xmlhttp.open("DELETE", baseTodoURL + "/" + id, false);
    xmlhttp.send();
}

// UPDATE Todo
function updateTodo(event, id, completed) {
    event.preventDefault();
    const params = {
        completed: completed
    }

    var xmlhttp = new XMLHttpRequest();
    //xmlhttp.addEventListener("load", createTodoReqListener);
    xmlhttp.open("PUT", baseTodoURL + "/" + id, false);
    xmlhttp.setRequestHeader("Content-Type", "application/json; charset=utf-8");
    xmlhttp.send(JSON.stringify(params));
    console.log(xmlhttp);
    if (xmlhttp.status != 201) {
        alert(`Error ${xmlhttp.status}: ${xmlhttp.response}`);
    } else {
        const todo = JSON.parse(xmlhttp.responseText);
        addSingleTodo(todo);
    }

}

window.onpaint = fetchTodos();
