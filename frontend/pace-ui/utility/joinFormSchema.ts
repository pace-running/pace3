import * as Yup from 'yup';

const requiredTShirtField = Yup.string().when('tshirt_toggle', {
  is: true,
  then: Yup.string().required('Bitte geben Sie die notwendigen Lieferinformationen an!')
});
Yup.string();

export const JoinFormSchema = Yup.object().shape({
  firstname: Yup.string()
    .min(2, 'Vorname muss mindestens zwei Zeichen enthalten!')
    .max(50, 'Vorname darf maximal 50 Zeichen enthalten!'),
  lastname: Yup.string()
    .min(2, 'Nachname muss mindestens zwei Zeichen enthalten!')
    .max(50, 'Nachname darf maximal 50 Zeichen enthalten!'),
  team: Yup.string(),
  email: Yup.string().email(),
  repeated_email: Yup.string().test({
    message: 'E-Mail Adressen müssen übereinstimmen!',
    test: function (value) {
      return value === this.parent.email;
    }
  }),
  starting_point: Yup.string().required('Bitte wählen Sie eine Option aus!'),
  running_level: Yup.string().required('Bitte wählen Sie eine Option aus!'),
  donation: Yup.number()
    .min(5, 'Die Spende muss mindestens 5€ betragen!')
    .required('Bitte geben Sie einen Spendenbetrag an!'),

  tshirt_toggle: Yup.boolean(),
  tshirt_model: Yup.string().when('tshirt_toggle', {
    is: true,
    then: Yup.string().required('Bitte geben Sie die Lieferadresse an!')
  }),
  tshirt_size: requiredTShirtField,
  country: requiredTShirtField,
  address_firstname: Yup.string().when('tshirt_toggle', {
    is: true,
    then: Yup.string()
      .min(2, 'Vorname muss mindestens zwei Zeichen enthalten!')
      .max(50, 'Vorname darf maximal 50 Zeichen enthalten!')
      .required('Bitte geben Sie die notwendigen Lieferinformationen an!')
  }),
  address_lastname: Yup.string().when('tshirt_toggle', {
    is: true,
    then: Yup.string()
      .min(2, 'Nachname muss mindestens zwei Zeichen enthalten!')
      .max(50, 'Nachname darf maximal 50 Zeichen enthalten!')
      .required('Bitte geben Sie die notwendigen Lieferinformationen an!')
  }),
  street_name: requiredTShirtField,
  house_number: requiredTShirtField,
  address_extra: Yup.string(),
  postal_code: requiredTShirtField,
  city: requiredTShirtField,

  tos_confirmed: Yup.boolean().required()
});

export type JoinFormValues = {
  firstname?: string;
  lastname?: string;
  team?: string;
  email?: string;
  repeated_email?: string;
  starting_point?: string;
  running_level?: string;
  donation: number;
  tshirt_cost: number;

  tshirt_toggle: boolean;
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

  tos_confirmed: boolean;
};
