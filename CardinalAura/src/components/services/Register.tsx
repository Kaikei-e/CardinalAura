import { ChangeEventHandler, useState } from "react";
import { SubmitButton } from "../elements/SubmitButton";
import { urlValidationSchema } from "../../lib/schema/url";

export const Register = () => {
  const [registerURL, setRegisterURL] = useState<string>("");

  const handleURLInput: ChangeEventHandler<HTMLInputElement> = (e) => {
    setRegisterURL(e.currentTarget.value);
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const isValid = urlValidationSchema.parse(registerURL);
    if (isValid) {
      console.log("valid");
      alert("valid");
    } else {
      console.log("invalid");
      alert("invalid");
    }
  };

  return (
    <div className="h-[100%] w-[100%] flex-col">
      <form onSubmit={handleSubmit}>
        <input
          type="url"
          placeholder="RSS feed link"
          onChange={handleURLInput}
        />
        <div className=" w-2/12 justify-center">
          <SubmitButton inputType="submit">
            Submit
          </SubmitButton>
        </div>
      </form>
    </div>
  );
};
