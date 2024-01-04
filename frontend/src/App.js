import React from 'react';
import './App.css';
import { useQuery, useMutation, QueryClient, QueryClientProvider } from 'react-query';
import TodoList from './TodoList';

const queryClient = new QueryClient();

const App = () => {
  return (
    <QueryClientProvider client={queryClient}>
      <div className='todo-app'>
      <TodoList />
      </div>
    </QueryClientProvider>
  );
};

export default App;