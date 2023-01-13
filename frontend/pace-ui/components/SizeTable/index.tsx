import React, { useState } from 'react';
import Carousel from 'react-bootstrap/Carousel';

const SizeTable: React.FC = () => {
  const [index, setIndex] = useState(0);

  const handleSelect = (selectedIndex: number) => {
    setIndex(selectedIndex);
  };

  return (
    <div>
      <h3 style={{ paddingTop: '0' }}>T-Shirt Größentabelle</h3>
      <p>alle Maße in cm</p>
      <Carousel variant='dark' indicators={false} activeIndex={index} onSelect={handleSelect}>
        <Carousel.Item>
          <div style={{ paddingBottom: '25px', textAlign: 'center' }}>
            <h2>Tailliert</h2>
            <div style={{ overflowX: 'auto' }}>
              <table className='size-table center' id='slimfit_table'>
                <thead>
                  <tr>
                    <th>Größe</th>
                    <th>Weite</th>
                    <th>Länge</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td>S</td>
                    <td>41</td>
                    <td>61</td>
                  </tr>
                  <tr>
                    <td>M</td>
                    <td>44</td>
                    <td>63</td>
                  </tr>
                  <tr>
                    <td>L</td>
                    <td>47</td>
                    <td>65</td>
                  </tr>
                  <tr>
                    <td>XL</td>
                    <td>50</td>
                    <td>67</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </Carousel.Item>
        <Carousel.Item>
          <div>
            <h2>Unisex</h2>
            <div style={{ overflowX: 'auto', marginBottom: '30px' }}>
              <table className='center size-table' id='unisex_table'>
                <thead>
                  <tr>
                    <th>Größe</th>
                    <th>Weite</th>
                    <th>Länge</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td>S</td>
                    <td>50</td>
                    <td>69</td>
                  </tr>
                  <tr>
                    <td>M</td>
                    <td>53</td>
                    <td>72</td>
                  </tr>
                  <tr>
                    <td>L</td>
                    <td>56</td>
                    <td>74</td>
                  </tr>
                  <tr>
                    <td>XL</td>
                    <td>58</td>
                    <td>76</td>
                  </tr>
                  <tr>
                    <td>XXL</td>
                    <td>62</td>
                    <td>78</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </Carousel.Item>
      </Carousel>
    </div>
  );
};

export default SizeTable;
