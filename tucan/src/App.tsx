import Button from '@mui/material/Button';
import { Link } from "react-router-dom";
import MiniDrawer from './MiniDrawer';

function App() {
  return (
    <>
    <MiniDrawer></MiniDrawer>
        <Link to="/login">Login</Link> |{" "}
      </>
  );
}

export default App;
