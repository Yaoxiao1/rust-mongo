import { Outlet, Link } from "react-router-dom";

const Layout = () => {
  return (
    <>
      <nav>
        <ul>
          <li>
            <Link to="/">Home</Link>
          </li>
          <li>
            <Link to="/update_user">UpdateUser</Link>
          </li>
          <li>
            <Link to="/generate">Generate</Link>
          </li>
          <li>
            <Link to="/insertPaper">InsertPaper</Link>
          </li>
          <li>
            <Link to="/insertUserInfo">insertUserInfo</Link>
          </li>
        </ul>
      </nav>

      <Outlet />
    </>
  );
};

export default Layout;
