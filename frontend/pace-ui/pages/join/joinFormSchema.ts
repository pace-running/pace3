import * as Yup from "yup";

const requiredTShirtField = Yup.string().when("tshirt_toggle", {
  is: true,
  then: Yup.string().required("Must enter email address"),
});

export const JoinFormSchema = Yup.object().shape({
  firstname: Yup.string().min(2, "Too Short!").max(50, "Too Long!"),
  lastname: Yup.string().min(2, "Too Short!").max(50, "Too Long!"),
  team: Yup.string(),
  email: Yup.string().email(),
  repeated_email: Yup.string().test({
    message: "Emails must be equals",
    test: function (value) {
      return value === this.parent.email;
    },
  }),
  starting_point: Yup.string().required("you have to fill"),
  running_level: Yup.string().required("you have to fill"),
  donation: Yup.number().min(5, "Too Short!").required("you have to fill"),

  tshirt_toggle: Yup.boolean(),
  tshirt_model: requiredTShirtField,
  tshirt_size: requiredTShirtField,
  country: requiredTShirtField,
  address_firstname: requiredTShirtField,
  address_lastname: requiredTShirtField,
  street_name: requiredTShirtField,
  house_number: requiredTShirtField,
  address_extra: requiredTShirtField,
  postal_code: requiredTShirtField,
  city: requiredTShirtField,

  tos_confirmed: Yup.boolean().required(),
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
