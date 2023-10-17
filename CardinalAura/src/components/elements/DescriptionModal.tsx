import { QuestionMarkCircleIcon } from "@heroicons/react/20/solid";
import { useEffect, useState } from "react";

type ModalProps = {
  isActive: boolean;
  onClose: () => void;
  description: string;
};

export const DescriptionModal = ({
  isActive,
  description,
  onClose,
}: ModalProps) => {
  const [isModalOpen, setIsModalOpen] = useState(false);

  useEffect(() => {
    setIsModalOpen(isActive);
  }, [isActive]);

  return (
    isModalOpen && (
      <div className="fixed inset-0 flex items-center justify-center z-50">
        <div className="relative transform overflow-hidden rounded-lg bg-white text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg">
          <div className="bg-white px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
            <div className="sm:flex sm:items-start">
              <QuestionMarkCircleIcon className="h-12 w-12 text-blue-400" />
              <div className="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
                <h3
                  className="text-base font-semibold leading-6 text-gray-900"
                  id="modal-title"
                >
                  What is this feature?
                </h3>
                <div className="mt-2">
                  <p className="text-md text-gray-700">{description}</p>
                </div>
              </div>
            </div>
          </div>
          <div className="bg-gray-50 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6">
            <button
              type="button"
              className="mt-3 inline-flex
            w-full justify-center rounded-md
          bg-white px-3 py-2
            text-sm font-semibold text-gray-900
            shadow-sm ring-1 ring-inset ring-gray-300
          hover:bg-gray-50 sm:mt-0 sm:w-auto"
              onClick={onClose}
            >
              Close
            </button>
          </div>
        </div>
      </div>
    )
  );
};
