import { Admin, Resource } from "react-admin";
import { authProvider } from "./auth/authProvider";
import { dataProvider } from "./dataProvider";
import { MessageList, MessageShow } from "./messages";

export default function App() {
  return (
    <Admin
      authProvider={authProvider}
      dataProvider={dataProvider}
      requireAuth
      title="OWEN Admin"
    >
      <Resource name="messages" list={MessageList} show={MessageShow} />
    </Admin>
  );
}
