import React from "react";

const SizeTable: React.FC = () => {
    return (
        <div>
            <div style={{paddingBottom: "25px", textAlign: "center"}}>
                <h2>Tailliert:</h2>
                <div style={{overflowX: "auto", justifyContent: "center"}}>
                <table className="size-table center" id="slimfit_table">
                    <tr>
                    <th>Size</th>
                    <th>Length</th>
                    <th>Width</th>
                    </tr>
                    <tr>
                        <td>S</td>
                        <td>61</td>
                        <td>41</td>
                    </tr>
                    <tr>
                        <td>M</td>
                        <td>63</td>
                        <td>44</td>
                    </tr>
                    <tr>
                        <td>L</td>
                        <td>65</td>
                        <td>47</td>
                    </tr>
                    <tr>
                        <td>XL</td>
                        <td>67</td>
                        <td>50</td>
                    </tr>
                </table>
                </div>
            </div>
            
            <div>
                <h2>Unisex:</h2>
                <div style={{overflowX: "auto"}}>
                <table className="center size-table" id="unisex_table">
                    <tr>
                    <th>Size</th>
                    <th>Length</th>
                    <th>Width</th>
                    </tr>
                    <tr>
                    <td>S</td>
                    <td>69</td>
                    <td>50</td>
                    </tr>
                    <tr>
                    <td>M</td>
                    <td>72</td>
                    <td>53</td>
                    </tr>
                    <tr>
                    <td>L</td>
                    <td>74</td>
                    <td>56</td>
                    </tr>
                    <tr>
                    <td>XL</td>
                    <td>76</td>
                    <td>58</td>
                    </tr>
                    <tr>
                    <td>XXL</td>
                    <td>78</td>
                    <td>62</td>
                    </tr>
                </table>
                </div>
            </div>
        </div>
    );
}

export default SizeTable;