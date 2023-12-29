import React, { useState } from 'react';
import { RiCloseCircleLine } from 'react-icons/ri';
import {TiEdit} from 'react-icons/ti';
import { RxUpdate } from "react-icons/rx";
const TodoItem = ({ todo, onDelete, onEdit }) => {
  const [isEditing, setIsEditing] = useState(false);
  const [editedTodo, setEditedTodo] = useState(todo.originalTodo);

  const handleEdit = () => {
    setIsEditing(true);
  };

  const handleUpdate = () => {
    onEdit(editedTodo);
    setIsEditing(false);
  };

  const handleCancelEdit = () => {
    setIsEditing(false);
  };

  return (
    <div className={`todo-row ${isEditing ? 'editing' : ''}`}>
      {isEditing ? (
        <>
          <input
            type="text"
            value={editedTodo}
            onChange={(e) => setEditedTodo(e.target.value)}
          />
          <div className='icons'>
            <RxUpdate className='update-icon' onClick={handleUpdate} />
            <RiCloseCircleLine className='delete-icon' onClick={handleCancelEdit} />
          </div>
        </>
      ) : (
        <>
          <span>{todo.originalTodo}</span>
          <div className='icons'>
            <TiEdit className='edit-icon' onClick={handleEdit} />
            <RiCloseCircleLine className='delete-icon' onClick={onDelete} />
          </div>
        </>
      )}
    </div>
  );
};


export default TodoItem;