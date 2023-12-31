import { ChangeEventHandler, useState } from "react";
import { SubmitButton } from "../elements/SubmitButton";
import { urlValidationSchema } from "../../lib/schema/url";
import { registerSingleFeedLink } from "../../lib/commands/feed/register";

export const Register = () => {
  const [registerURL, setRegisterURL] = useState<string>("");

  const handleURLInput: ChangeEventHandler<HTMLInputElement> = (e) => {
    setRegisterURL(e.currentTarget.value);
  };

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    e.stopPropagation();

    const isValid = await urlValidationSchema.parse(registerURL);
    if (isValid) {
      const response = await registerSingleFeedLink(registerURL);
      if (response) {
        alert(response.url);
      } else if (typeof response === "string") {
        alert(response);
      }
    } else {
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
          <SubmitButton inputType="submit">Submit</SubmitButton>
        </div>
      </form>
    </div>
  );
};
