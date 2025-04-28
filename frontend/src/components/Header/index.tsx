import { HeaderContainer } from "./styles";
import { NavLink } from "react-router-dom";

export function Header() {
	return (
		<HeaderContainer>
			<nav>
				<NavLink to="/" title="Home">Home</NavLink>
			</nav>
		</HeaderContainer>
	)
}
