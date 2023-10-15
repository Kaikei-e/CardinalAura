import { createBrowserRouter } from "react-router-dom";
import "./App.css";
import { Home } from "./routes/Home";

export const AllRoutes = createBrowserRouter([
  {
    path: "/",
    element: <App />,
  },
  {
    path: "/home",
    element: <Home />,
  },
]);

function App() {
  return (
    <div className="base p-8 h-screen justify-between flex flex-col">
      <div className="justify-center">
        <h1 className="h1-style mt-20 pt-10vh flex justify-center text-center">
          Cardinal Aura
        </h1>
        <p className="mt-2 font-semibold">Explore: by curiosity.</p>
      </div>
      <div className=" m-10 flex flex-row justify-end">
        <div className="w-3/12">
          <button
            className="w-11/12 p-2
          border-2 border-white
          rounded-md
          hover:bg-white hover:text-gray-700
          text-white font-semibold"
            onClick={() => {
              AllRoutes.navigate("/home");
            }}
          >
            Explore
          </button>
        </div>
      </div>
    </div>
  );
}

export default App;
