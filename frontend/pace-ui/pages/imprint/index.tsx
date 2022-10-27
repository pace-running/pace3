import type { NextPage } from 'next';
import BaseLayout from '../../components/Layout/baseLayout';

const Imprint: NextPage = () => {
  return (
    <BaseLayout pageTitle='Impressum'>
      <div className='container'>
        <h1>Impressum</h1>
        Anbieterangaben gem. § 5 TMG
        <br />
        <h2 style={{fontSize: '1.5rem'}}>Fußball-Club St. Pauli v. 1910 e.V.</h2>
        - Abteilung: Marathon -<br />
        Harald-Stender-Platz 1<br />
        20359 Hamburg
        <br />
        Telefon: 040 - 31 78 74 0<br />
        E-Mail: abteilungsleitung@fcstpauli-marathon.de
        <br />
        Der Fußball-Club St. Pauli v. 1910 e.V. wird vertreten durch den Vorstand
        <br />
        Oke Göttlich (Präsident), Christiane Hollander, Carsten Höltkemeyer, Joachim Pawlik, Jochen Winand
        (Vizepräsidenten)
        <br />
        Registergericht: Amtsgericht Hamburg
        <br />
        Vereinsregisternummer: VR1884
        <br />
      </div>
    </BaseLayout>
  );
};

export default Imprint;
