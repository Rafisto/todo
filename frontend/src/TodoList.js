import React, { useState } from 'react';
import TodoItem from './TodoItem';
import { MdAddCircleOutline } from "react-icons/md";
const TodoList = () => {
  const [todos, setTodos] = useState([]);
  const [inputValue, setInputValue] = useState('');

  const addTodo = () => {
    if (inputValue.trim() !== '') {
      const newTodo = {
        originalTodo: inputValue,
        editedTodo: inputValue,
      };
      setTodos((prevTodos) => [...prevTodos, newTodo]);
      setInputValue('');
    }
  };

  const editTodo = (index, editedTodo) => {
    const updatedTodos = [...todos];
    updatedTodos[index] = {
      originalTodo: editedTodo,
      editedTodo: editedTodo,
    };
    setTodos(updatedTodos);
  };


  const deleteTodo = (todo) => {
    setTodos((prevTodos) => prevTodos.filter((t) => t !== todo));
  };

  return (
    <div className="todo-list">
      <h1>Todo List</h1>  
      <div>
        <form
          className='todo-form'
          onSubmit={(e) => {
            e.preventDefault();
            addTodo();
          }}
        >
          <input
            type="text"
            placeholder="What you need to do?"
            value={inputValue}
            className='todo-input'
            onChange={(e) => setInputValue(e.target.value)}
          />
          
          <button type="submit" className='todo-button'>Add</button>
        </form>
      </div>

      <div>
      {todos.map((todo, index) => {
  return (
    <TodoItem
      key={index}
      todo={todo}
      onDelete={() => deleteTodo(todo)}
      onEdit={(editedTodo) => editTodo(index, editedTodo)}
    />
  );
})}
      </div>
    </div>
  );
};

export default TodoList;