import { ReactNode } from "react";

type buttonProps = {
  inputType: string;
  children: ReactNode;
};

export const SubmitButton = ({ inputType, children }: buttonProps) => {
  return (
    <div className=" h-14 justify-center bg-gray-50 rounded-md px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6">
      <button
        type={inputType === "button" ? "button" : "submit"}
        className="
      mt-3 inline-flex align-middle
      w-full justify-center rounded-md
      bg-white px-3 py-2
      text-sm font-semibold text-gray-900
      shadow-sm ring-1 ring-inset ring-gray-300
      hover: sm:mt-0 sm:w-auto"
      >
        {children}
      </button>
    </div>
  );
};
