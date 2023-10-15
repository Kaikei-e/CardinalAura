export const Home = () => {
  return (
    <div className="h-screen p-8 text-gray-700 bg-red-300">
      <h1 className="h1-style pt-3">Home</h1>
      <p className="font-semibold text-xl text-gray-700">Welcome to Cardinal Aura. Curiosity guides you.</p>
      <div className="h-[calc(100%-5rem)] w-full p-8 grid grid-cols-3 gap-4 ">
        <div className="w-full h-full bg-yellow-100 rounded-md flex flex-col justify-start  text-center">
          <p className="text-2xl text-center underline">Insight</p>
          <p className="mt-2">Insight suggests what you need from accumulated RSS feeds.</p>
        </div>
        <div className="w-full h-full bg-yellow-100 rounded-md flex flex-col justify-start  text-center">
          <p className="text-2xl text-center underline">Storage</p>
          <p className="mt-2">Storage helps you effortlessly search through the feeds you've gathered, ensuring you get the most from what you've collected.</p>
        </div>
        <div className="w-full h-[95%] bg-yellow-100 rounded-md flex flex-col row-span-3 justify-start text-center">
          <p className="text-2xl text-center underline">Stream</p>
          <p className="mt-2">Stream displays the RSS feeds you've acquired, showcasing them in the latest order.</p>
        </div>
        <div className="col-span-2 bg-yellow-100 rounded-md ...">07</div>
      </div>
    </div>
  );
};
