import * as Yup from 'yup';



export const EditRunnerSchema = Yup.object().shape({
  firstname: Yup.string()
    .min(2, 'Vorname muss mindestens zwei Zeichen enthalten!')
    .max(50, 'Vorname darf maximal 50 Zeichen enthalten!'),
  lastname: Yup.string()
    .min(2, 'Nachname muss mindestens zwei Zeichen enthalten!')
    .max(50, 'Nachname darf maximal 50 Zeichen enthalten!'),
  team: Yup.string(),
  email: Yup.string().email(),
//   starting_point: Yup.string().required('Bitte wählen Sie eine Option aus!'),
//   running_level: Yup.string().required('Bitte wählen Sie eine Option aus!'),
//   donation: Yup.number()
//     .min(5, 'Die Spende muss mindestens 5€ betragen!')
//     .required('Bitte geben Sie einen Spendenbetrag an!'),

  is_tshirt_booked: Yup.boolean(),
  tshirt_model: Yup.string(),
  tshirt_size: Yup.string(),
  country: Yup.string(),
  address_firstname: Yup.string(),
  address_lastname: Yup.string(),
  street_name: Yup.string(),
  house_number: Yup.string(),
  address_extra: Yup.string(),
  postal_code: Yup.string(),
  city: Yup.string(),

  start_number: Yup.string(),
  verification_code: Yup.string(),
  reason_for_payment: Yup.string(),
  payment_status: Yup.boolean(),
  delivery_status: Yup.string(),
  
});

export type EditRunnerValues = {
  firstname?: string;
  lastname?: string;
  team?: string;
  email?: string;
  starting_point?: string;
  running_level?: string;
  donation: number;

  is_tshirt_booked: boolean;
  tshirt_model?: string;
  tshirt_size?: string;
  country?: string;
  address_firstname?: string;
  address_lastname?: string;
  street_name?: string;
  house_number?: string;
  address_extra?: string;
  postal_code?: string;
  city?: string;

  start_number: string;
  verification_code: string;
  reason_for_payment: string;
  payment_status: boolean;
  delivery_status?: string;
};