import {
  Heading,
  IconButton,
  VStack,
  useColorMode,
  useToast,
} from "@chakra-ui/react";
import TaskList from "./components/tasks";
import AddTask from "./components/AddTask";
import { FaSun, FaMoon } from "react-icons/fa";
import { useState, useEffect } from "react";

// const WORMHOLE_URL = "https://mauve-sun-9136.cosmonic.app/";
const WORMHOLE_URL = "/";

function App() {
  const toast = useToast();

  const [tasks, setTasks] = useState(
    () => undefined
  );

  const requestOptions = {
    method: 'GET',
    headers: { 'Content-Type': 'application/json' }
  };

  if (!tasks) {
    fetch(`${WORMHOLE_URL}api`, requestOptions)
      .then(data => data.json())
      .then(tasks => setTasks(tasks));
  }

  function deleteTask(url) {
    const requestOptions = {
      method: 'DELETE',
      headers: { 'Content-Type': 'application/json' }
    };
    fetch(`${url}`, requestOptions)
      .then(data => console.log(data));

    const newTasks = tasks.filter((task) => {
      return task.url !== url;
    });
    setTasks(newTasks);
  }

  function deleteTaskAll() {
    const requestOptions = {
      method: 'DELETE',
      headers: { 'Content-Type': 'application/json' }
    };
    fetch(`${WORMHOLE_URL}api`, requestOptions)
      .then(data => console.log(data));
    setTasks([]);
  }

  function checkTask(url) {
    const newTasksCheck = tasks.map((task, index, array) => {
      if (task.url === url) {
        task.check = !task.check;
        task.completed = !task.completed;
        const requestOptions = {
          method: 'PATCH',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(task)
        };
        fetch(`${url}`, requestOptions)
          .then(data => console.log(data));
      }
      return task;
    });
    setTasks(newTasksCheck);
  }

  function updateTask(url, title, onClose) {
    const info = title.trim();

    if (!info) {
      toast({
        title: "Enter your task",
        position: "top",
        status: "warning",
        duration: 2000,
        isClosable: true,
      });

      return;
    }

    const newTasksUpdate = tasks.map((task, index, array) => {
      if (task.url === url) {
        task.title = info;
        task.check = false;
        const requestOptions = {
          method: 'PATCH',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(task)
        };
        fetch(`${url}`, requestOptions)
          .then(data => console.log(data));
      }
      return task;
    });

    setTasks(newTasksUpdate);

    onClose();
  }

  function addTask(task) {
    const requestOptions = {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(task)
    };
    fetch(`${WORMHOLE_URL}api`, requestOptions)
      .then(data => console.log(data));
    setTasks([...tasks, task]);
  }

  const { colorMode, toggleColorMode } = useColorMode();

  return (
    <VStack p={4} minH='100vh' pb={28}>
      <IconButton
        icon={colorMode === "light" ? <FaSun /> : <FaMoon />}
        isRound='true'
        size='md'
        alignSelf='flex-end'
        onClick={toggleColorMode}
        aria-label='toogle-dark-mode'
      />

      <Heading
        p='5'
        fontWeight='extrabold'
        size='xl'
        bgGradient='linear(to-r, #00C389, #00F6AD)'
        bgClip='text'
      >
        Todo list
      </Heading>
      <AddTask addTask={addTask} />
      <TaskList
        tasks={tasks}
        updateTask={updateTask}
        deleteTask={deleteTask}
        deleteTaskAll={deleteTaskAll}
        checkTask={checkTask}
      />
    </VStack>
  );
}

export default App;
