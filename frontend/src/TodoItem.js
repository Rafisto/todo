import React, { useState } from 'react';
import { RiCloseCircleLine } from 'react-icons/ri';
import { TiEdit } from 'react-icons/ti';
import { RxUpdate } from 'react-icons/rx';

const TodoItem = ({ todo, onDelete, onEdit }) => {
  const [isEditing, setIsEditing] = useState(false);
  const [editedTodo, setEditedTodo] = useState(todo.title);

  const handleEdit = () => {
    setIsEditing(true);
  };

  const handleUpdate = () => {
    onEdit({
      task_id: todo.task_id,
      title: editedTodo,
      description: editedTodo,
      completed: todo.completed,
    });
    setIsEditing(false);
  };
  const handleCancelEdit = () => {
    setIsEditing(false);
  };

  const handleCheckboxChange = () => {
    onEdit({
      task_id: todo.task_id,
      title: todo.title,
      description: todo.description,
      completed: !todo.completed,
    });
  };

  const isCompleted = todo.completed;
  const isDue = todo.due_date && new Date(todo.due_date) < new Date();
  const rowClassName = `todo-row ${isEditing ? 'editing' : ''} ${isCompleted ? 'completed' : ''} ${isDue ? 'due' : ''}`;
  return (
    <div className={rowClassName}>
      {isEditing ? (
        <>
          <input
            type="text"
            value={editedTodo}
            onChange={(e) => setEditedTodo(e.target.value)}
          />
          <div className="icons">
            <RxUpdate className="update-icon" onClick={handleUpdate} />
            <RiCloseCircleLine
              className="delete-icon"
              onClick={handleCancelEdit}
            />
          </div>
        </>
      ) : (
        <>
          <input
            type="checkbox"
            checked={todo.completed}
            onChange={handleCheckboxChange}
          />
          <span>{todo.title}</span>
          <div className="icons">
            <TiEdit className="edit-icon" onClick={handleEdit} />
            <RiCloseCircleLine className="delete-icon" onClick={onDelete} />
          </div>
        </>
      )}
    </div>
  );
};

export default TodoItem;