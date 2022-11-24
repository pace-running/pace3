export const startingOptions = [
  { label: 'in Hamburg bei der Alster vor Ort', value: 'hamburg' },
  { label: 'woanders', value: 'other' }
];

export const runningLevelOptions = [
  { label: 'Ich laufe selten', value: 'rarely' },
  { label: 'Ich laufe gelegentlich bis regelmäßig', value: 'sometimes' },
  { label: 'Ich laufe häufig und ambitioniert', value: 'often' }
];

export const modelOptions = [
  { label: 'Unisex', value: 'unisex' },
  { label: 'Tailliert', value: 'slimfit' }
];

export const countryOptions = [
  { label: 'Deutschland', value: 'Deutschland' },
  { label: 'Afghanistan', value: 'Afghanistan' },
  { label: 'Albania', value: 'Albania' }
];

export const getSizeOptions = (modelOptionValue: string | undefined): { label: string; value: string }[] => {
  const defaultSizes = [
    { label: 'S', value: 's' },
    { label: 'M', value: 'm' },
    { label: 'L', value: 'l' },
    { label: 'XL', value: 'xl' }
  ];

  if (modelOptionValue === 'unisex') {
    return [...defaultSizes, { label: 'XXL', value: 'xxl' }];
  }

  return defaultSizes;
};
