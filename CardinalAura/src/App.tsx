import "./App.css";

function App() {
  return (
    <div className="container h-screen justify-between flex flex-col">
      <div className="justify-center ">
        <h1 className=" text-3xl font-semibold m-0 pt-10vh flex justify-center text-center">
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
          >
            Explore
          </button>
        </div>
      </div>
    </div>
  );
}

export default App;
