import React, { useState } from 'react';
import { useQuery, useMutation} from 'react-query';
import { useQueryClient } from 'react-query'; 
import TodoItem from './TodoItem';

const TodoList = () => {
  const queryClient = useQueryClient();
  const [inputValue, setInputValue] = useState('');
  const [dueDate, setDueDate] = useState(null); 
  const backendUrl = process.env.BACKEND_URL || 'http://backend:8000';
  // Fetch tasks
  const { data: todos = [], isLoading } = useQuery('todos', async () => {
    const response = await fetch('BACKEND_URL/todo');
    const responseData = await response.json();
    return responseData;
  });
  // Add task mutation
  const addTaskMutation = useMutation((newTodo) =>
    fetch('${backendUrl}/todo', {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(newTodo),
    }).then((response) => response.json()),
    {
      onSuccess: () => {
        queryClient.invalidateQueries('todos');
      },
    }
  );

  const addTodo = () => {
    if (inputValue.trim() !== '') {
      const newTodo = {
        completed: false,
        description: inputValue,
        title: inputValue,
        due_date: dueDate,
      };

      addTaskMutation.mutate(newTodo, {
        onSuccess: () => {
          setInputValue('');
          setDueDate('');
        },
      });
    }
  };

  // Update task mutation
  const updateTaskMutation = useMutation(({ taskId, updatedTodo }) =>
    fetch('${backendUrl}/todo/${taskId}', {
      method: 'PATCH',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(updatedTodo),
    }).then((response) => response.json()),
    {
      onSuccess: () => {
        queryClient.invalidateQueries('todos');
      },
    }
  );

  const editTodo = (editedTodo) => {
    
    const todoToUpdate = todos.find((todo) => todo.task_id === editedTodo.task_id);
  
    if (!todoToUpdate || !editedTodo || !editedTodo.task_id) {
      console.error('Invalid todo or editedTodo');
      return;
    }
  
  
    const updatedTodo = {
      ...todoToUpdate,
      title: editedTodo.title,
      description: editedTodo.description,
      completed: editedTodo.completed,
    };
  
  
    updateTaskMutation.mutate({
      taskId: editedTodo.task_id,
      updatedTodo,
    });
  };

  // Delete task mutation
  const deleteTaskMutation = useMutation((taskId) =>
  fetch('${backendUrl}/todo/${taskId}', {
    method: 'DELETE',
  })
);

const deleteTodo = (todo) => {
  deleteTaskMutation.mutate(todo.task_id, {
    onSuccess: () => {
      queryClient.invalidateQueries('todos');
    },
  });
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
        <input
          type="date"
          value={dueDate}
          onChange={(e) => setDueDate(e.target.value)}
          className='todo-input-date' 
        />


          <button type="submit" className='todo-button'>Add</button>
        </form>
      </div>
      <div>
      {isLoading ? (
  <p>Loading...</p>
) : (
  todos.map((todo) => (
    <TodoItem
    key={todo.task_id}
    todo={todo}
    onDelete={() => deleteTodo(todo)}
    onEdit={(editedTodo) => editTodo({ task_id: todo.task_id, ...editedTodo })}
  />
  ))
)}
      </div>
    </div>
  );
};

export default TodoList;