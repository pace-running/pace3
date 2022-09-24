import type {NextPage} from "next";
import BaseLayout from "../../components/Layout/baseLayout";
import TextInput from "../../components/TextInput";
import Button from "../../components/Button";
import axios from "axios";
import router from "next/router";
import React from "react";


const submitForm = async (event: React.SyntheticEvent) => {
    event.preventDefault()
    const target = event.target as typeof event.target & {
        username: { value: string };
        password: { value: string };
    };
    let loginData = {
        username: target.username.value,
        password: target.password.value
    }
    await axios.post('http://localhost:8080/api/admin/login', loginData).then((response => {
        console.log(response)
        router.push('/admin')
    }))
}

const Login: NextPage = () => {
    return (
        <BaseLayout pageTitle="Admin Login">
            <form onSubmit={submitForm}>
                <div className="container">
                    <h1>Login</h1>
                    <TextInput type={"text"} name={"username"} label={"Username"}/>
                    <TextInput type={"password"} name={"password"} label={"Passwort"}/>
                    <Button name={"submitButton"} label={"Login"} type={"submit"} styling={"brownbg"}/>
                </div>
            </form>
        </BaseLayout>
    );
};

export default Login;