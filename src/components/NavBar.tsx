import styled from "styled-components";

const StyledLink = styled.a`
  padding: 5px;
`;
const NavBar = () => {
  return (
    <>
      <StyledLink href="/">Home</StyledLink>
      <StyledLink href="/primer">Primer Generation</StyledLink>
      <StyledLink href="/encode">Encode File</StyledLink>
      <StyledLink href="/decode">Decode File</StyledLink>
    </>
  );
};

export default NavBar;
