import { ChangeEventHandler, ReactNode } from "react";

type CheckboxProps = {
  name: string;
  label: string;
  role?: "switch";
  check: boolean;
  onChange?: ChangeEventHandler;
  rest?: ReactNode;
};

const Checkbox: React.FC<CheckboxProps> = (props) => {
  return (
    <div
      className={props.role ? "form-check form-switch mb-3" : "form-check mb-3"}
    >
      <input
        className="form-check-input"
        type="checkbox"
        value=""
        id={props.name}
        onChange={props.onChange}
        role={props.role ?? undefined}
        checked={props.check}
      />
      <label className="form-check-label" htmlFor={props.name}>
        {props.label}
        {props.rest}
      </label>
    </div>
  );
};

export default Checkbox;
