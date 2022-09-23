import { ChangeEventHandler } from "react";

type Option = {
  label: string;
  value: string | number;
};

type DropdownProps = {
  name: string;
  label: string;
  options: Option[];
  selected?: string | number;
  onChange?: ChangeEventHandler;
  valid?: boolean;
  errorMessage?: string;
};

const Dropdown: React.FC<DropdownProps> = (props) => {
  return (
    <div className="mb-3">
      <label htmlFor={props.name} className="form-label">
        {props.label}
      </label>
      <select
        name={props.name}
        id={props.name}
        className="form-select"
        aria-label={props.label}
        onChange={props.onChange}
      >
        <option style={{ display: "none" }}>Bitte auswählen</option>
        {props.options.map((option) => {
          return (
            <option
              //   selected={option.value === props.selected}
              value={option.value}
              key={option.label}
            >
              {option.label}
            </option>
          );
        })}
      </select>
      {!props.valid && (
        <div
          style={{ display: !props.valid ? "block" : "none" }}
          className="invalid-feedback"
        >
          {props.errorMessage}
        </div>
      )}
    </div>
  );
};

export default Dropdown;
