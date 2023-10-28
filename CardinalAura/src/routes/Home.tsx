import { QuestionMarkCircleIcon } from "@heroicons/react/20/solid";
import { DescriptionModal } from "../components/elements/DescriptionModal";
import { useState } from "react";
import { Stream } from "../components/services/Stream";
import { Register } from "../components/services/Register";

export const Home = () => {
  const [isStreamModalOpen, setIsStreamModalOpen] = useState(false);
  const [isStorageModalOpen, setIsStorageModalOpen] = useState(false);
  const [isInsightModalOpen, setIsInsightModalOpen] = useState(false);

  return (
    <div className="h-screen p-8 text-gray-700 bg-red-300">
      <h1 className="h1-style pt-3">Home</h1>
      <p className="font-semibold text-xl text-gray-700">
        Welcome to Cardinal Aura. Curiosity guides you.
      </p>
      <div className="h-[calc(100%-5rem)] w-full p-8 grid grid-cols-3 gap-4 ">
        <div className="w-full h-full bg-yellow-100 rounded-md flex flex-col justify-start  text-center">
          <div className="flex flex-row justify-center">
            <p className="text-2xl text-center underline">Insight</p>
            <button
              onClick={() => {
                setIsInsightModalOpen(true);
              }}
            >
              <QuestionMarkCircleIcon className="h-6 w-6 text-blue-400" />
            </button>
            <DescriptionModal
              description={insightDescription}
              isActive={isInsightModalOpen}
              onClose={() => {
                setIsInsightModalOpen(false);
              }}
            />
          </div>
        </div>
        <div className="w-full h-full bg-yellow-100 rounded-md flex flex-col justify-start  text-center">
          <div className="flex flex-row justify-center">
            <p className="text-2xl text-center underline">Storage</p>
            <button
              onClick={() => {
                setIsStorageModalOpen(true);
              }}
            >
              <QuestionMarkCircleIcon className="h-6 w-6 text-blue-400" />
            </button>
            <DescriptionModal
              description={storageDescription}
              isActive={isStorageModalOpen}
              onClose={() => {
                setIsStorageModalOpen(false);
              }}
            />
          </div>
        </div>
        <div className="w-full h-[100%] bg-yellow-100 rounded-md flex flex-col row-span-3 justify-start text-center">
          <div className="flex flex-row justify-center  inset-0 z-10 ">
            <p className="text-2xl text-center underline">Stream</p>
            <button
              onClick={() => {
                setIsStreamModalOpen(true);
              }}
            >
              <QuestionMarkCircleIcon className="h-6 w-6 text-blue-400" />
            </button>
          </div>
          <Stream />
          {isStreamModalOpen && (
            <DescriptionModal
              description={streamDescription}
              isActive={isStreamModalOpen}
              onClose={() => {
                setIsStreamModalOpen(false);
              }}
            />
          )}
        </div>
        <div className="col-span-2 bg-yellow-100 rounded-md ...">
          <p>Something</p>
          <Register />
        </div>
      </div>
    </div>
  );
};

const insightDescription =
  "Insight suggests what you need from accumulated RSS feeds.";
const storageDescription =
  "Storage helps you effortlessly search through the feeds you've gathered, ensuring you get the most from what you've collected.";
const streamDescription =
  "Stream is a live feed of your collection, allowing you to see what you've gathered in real time.";
