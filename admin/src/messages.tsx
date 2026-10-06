import {
  Datagrid,
  List,
  NumberField,
  SelectInput,
  Show,
  SimpleShowLayout,
  TextField,
} from "react-admin";

const statusChoices = [
  { id: "unprocessed", name: "unprocessed" },
  { id: "reserved", name: "reserved" },
  { id: "processed", name: "processed" },
  { id: "rejected", name: "rejected" },
];

const messageFilters = [
  <SelectInput
    key="processingStatus"
    source="processingStatus"
    choices={statusChoices}
    alwaysOn
    emptyText="unprocessed"
  />,
];

export const MessageList = () => (
  <List
    filters={messageFilters}
    sort={{ field: "createdTimestamp", order: "DESC" }}
  >
    <Datagrid rowClick="show" bulkActionButtons={false}>
      <TextField source="messageFolder" />
      <TextField source="processingStatus" />
      <NumberField source="createdTimestamp" />
      <NumberField source="updatedTimestamp" />
      <TextField source="owenInstance" />
    </Datagrid>
  </List>
);

export const MessageShow = () => (
  <Show>
    <SimpleShowLayout>
      <TextField source="id" />
      <TextField source="messageFolder" />
      <TextField source="processingStatus" />
      <NumberField source="createdTimestamp" />
      <NumberField source="updatedTimestamp" />
      <TextField source="owenInstance" />
    </SimpleShowLayout>
  </Show>
);
