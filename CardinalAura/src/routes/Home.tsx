export const Home = () => {
  return (
    <div className="h-screen p-8 text-gray-700 bg-red-300">
      <h1 className="h1-style pt-3">Home</h1>
      <p className="font-semibold text-gray-700">Welcome to Cardinal Aura</p>
      <div className="h-11/12 w-full p-8 grid grid-cols-3 gap-4 ">
        <div className="w-full h-full bg-yellow-100">
          <p className="text-2xl">Welcome to Cardinal Aura!</p>
        </div>
      </div>
    </div>
  );
};
