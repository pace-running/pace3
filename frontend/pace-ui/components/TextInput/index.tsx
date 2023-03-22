import React, { ChangeEvent, ChangeEventHandler, useEffect, useState } from 'react';

type InputProps = {
  type: 'email' | 'text' | 'number' | 'password';
  placeholder?: string;
  value?: string | number;
  name: string;
  label: string;
  helperLabel?: string;
  default?: string | number;
  onChange?: ChangeEventHandler;
  valid?: boolean;
  errorMessage?: string;
};
const TextInput: React.FC<InputProps> = props => {
  const [value, setValue] = useState(props.value || '');
  const onChange: ChangeEventHandler = (event: ChangeEvent<HTMLInputElement>) => {
    setValue(event.target.value);
    if (props.onChange !== undefined) {
      props.onChange(event);
    }
  };

  useEffect(() => {
    setValue(props.value || '');
  }, [props]);

  return (
    <div className='mb-3'>
      {props.helperLabel && <p style={{ marginBottom: '1px' }}>{props.helperLabel}</p>}
      <label htmlFor={props.name + '_input'} className='form-label'>
        {props.label}
      </label>
      <div className='input-group'>
        <input
          id={props.name + '_input'}
          value={value}
          onChange={onChange}
          type={props.type}
          className='form-control'
          name={props.name}
          placeholder={props.placeholder}
          defaultValue={props.default}
        />
      </div>
      {!props.valid && (
        <div style={{ display: !props.valid ? 'block' : 'none' }} className='invalid-feedback'>
          {props.errorMessage}
        </div>
      )}
    </div>
  );
};

export default TextInput;
