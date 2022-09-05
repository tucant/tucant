import Button from '@mui/material/Button';
import { Link } from "react-router-dom";

function App() {
  return (
    <>
    <Button variant="contained">Hello World</Button>
    <nav
        style={{
          borderBottom: "solid 1px",
          paddingBottom: "1rem",
        }}
      >
        <Link to="/login">Login</Link> |{" "}
      </nav>
      </>
  );
}

export default App;
