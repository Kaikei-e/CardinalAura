import { SubmitButton } from "../elements/SubmitButton";

export const Register = () => {
  return (
    <div className="h-[100%] w-[100%] flex-col">
      <form>
        <input type="url" placeholder="RSS feed link" />
        <div className=" w-2/12 justify-center">
          <SubmitButton inputType="button">Submit</SubmitButton>
        </div>
      </form>
    </div>
  );
};
