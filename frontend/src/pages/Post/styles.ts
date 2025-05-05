import styled from 'styled-components'

export const PostContainer = styled.div`
	max-width: 76rem;
	height: 100%;
	width: 100%;
	margin: 5rem auto;
	padding: 2.5rem;

	border-radius: 3px;
  border: 1px solid ${props => props.theme['secondary-900']};
  background: ${props => props.theme['secondary-950']};

  .titleArea {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
  }

  a {
    text-decoration: none;
    color: ${props => props.theme['secondary']};
  }

  article {
    margin: 2rem;
    font-size: 18px;
    line-height: 2;
  }
`
