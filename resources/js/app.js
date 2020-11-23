const todoInput = document.querySelector('.todo_input');
const todoButton = document.querySelector('.todo_button');
const todoList = document.querySelector('.todo_list');

todoButton.addEventListener("click", addTodo)
todoList.addEventListener("click", deleteCheck)

function addTodo(event) {
    event.preventDefault();
    const todoDiv = document.createElement('div');
    todoDiv.classList.add('todo_element');

    const completedButton = document.createElement('button');
    completedButton.innerHTML = '<i class="material-icons">check_circle_outline</i>';
    completedButton.classList.add('complete_btn')
    todoDiv.appendChild(completedButton);

    const newTodo = document.createElement('li');
    newTodo.innerText = todoInput.value;
    newTodo.classList.add('todo_item');
    todoDiv.appendChild(newTodo);
    if(todoInput.value === ""){
        return null
    }

    const deleteButton = document.createElement('button');
    deleteButton.innerHTML = '<i class="material-icons">delete</i>';
    deleteButton.classList.add('delete_btn')
    todoDiv.appendChild(deleteButton);

    todoList.appendChild(todoDiv);
    todoInput.value = ""
}

/*
<div class="todo">
    <button class="complete_btn">
        <i class="material-icons">check_circle_outline</i>
    </button>
    <li class="todo_item">
        Buy Milk
    </li>
    <button class="delete_btn">
        <i class="material-icons">delete</i>
    </button>
</div>
*/

function deleteCheck(e) {
    const item = e.target;
    // Delete todo
    if (item.classList[0] === "delete_btn") {
        const todo = item.parentElement;
        todo.addEventListener('transitionend', function () {
            todo.remove()
        })
    }
    // Complete todo
    if (item.classList[0] === "todo_item" || item.classList[0] === "complete_btn") {
        const todo = item.parentElement;
        todo.classList.toggle("completed-item")
        todo.querySelector('.complete_btn').children[0].classList.toggle("md-completed")
    }
}
