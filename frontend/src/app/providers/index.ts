import compose from "compose-function";
import { withRouter } from "./with-router";
import { withUi } from "./with-ui";
import {getWhitedList} from "../../proccesess/getWhitedList";

export const withProviders = compose(
   withRouter,
   withUi,
   getWhitedList
);