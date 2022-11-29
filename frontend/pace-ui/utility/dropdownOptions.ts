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

export const euCountryOptions = [
  { label: 'Belgien', value: 'Belgien' },
  { label: 'Bulgarien', value: 'Bulgarien' },
  { label: 'Dänemark', value: 'Dänemark' },
  { label: 'Estland', value: 'Estland' },
  { label: 'Finnland', value: 'Finnland' },
  { label: 'Frankreich', value: 'Frankreich' },
  { label: 'Griechenland', value: 'Griechenland' },
  { label: 'Irland', value: 'Irland' },
  { label: 'Italien', value: 'Italien' },
  { label: 'Lettland', value: 'Lettland' },
  { label: 'Litauen', value: 'Litauen' },
  { label: 'Luxemburg', value: 'Luxemburg' },
  { label: 'Malta', value: 'Malta' },
  { label: 'Niederlande', value: 'Niederlande' },
  { label: 'Österreich', value: 'Österreich' },
  { label: 'Polen', value: 'Polen' },
  { label: 'Portugal', value: 'Portugal' },
  { label: 'Rumänien', value: 'Rumänien' },
  { label: 'Schweden', value: 'Schweden' },
  { label: 'Slowakei', value: 'Slowakei' },
  { label: 'Slowenien', value: 'Slowenien' },
  { label: 'Spanien', value: 'Spanien' },
  { label: 'Tschechische Republik', value: 'Tschechische Republik' },
  { label: 'Ungarn', value: 'Ungarn' },
  { label: 'Zypern', value: 'Zypern' }
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

export const regionOptions = [
  { label: 'Deutschland', value: 'de' },
  { label: 'EU-Ausland', value: 'eu' },
  { label: 'Nicht-EU Ausland', value: 'non-eu' }
];
