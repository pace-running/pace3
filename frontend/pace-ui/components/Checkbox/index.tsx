import React, { ChangeEventHandler, ReactNode } from 'react';

type CheckboxProps = {
  name: string;
  label: string;
  role?: 'switch';
  check: boolean;
  onChange?: ChangeEventHandler;
  rest?: ReactNode;
  testID?: string;
};

const Checkbox: React.FC<CheckboxProps> = props => {
  return (
    <div className={props.role ? 'form-check form-switch mb-3' : 'form-check mb-3'}>
      <input
        className='form-check-input brown-checked'
        type='checkbox'
        value=''
        name={props.name}
        id={props.name}
        onChange={props.onChange}
        role={props.role ?? undefined}
        checked={props.check}
        data-testid={props.testID}
      />
      <label className='form-check-label' htmlFor={props.name}>
        {props.label}
        {props.rest}
      </label>
    </div>
  );
};

export default Checkbox;
