import logo from '../logo.png';
import { Center } from "../components/Center";

const Home = () => {
  return (
    <div>
      <h1>nuCloud</h1>
      <Center>
      <img src={logo} alt="Logo" />
      </Center>
    </div>
  );
};

export default Home;
