import { Link } from 'react-router-dom'
import styled from 'styled-components'

export const PostContainer = styled.main`
  max-width: 76rem;
  height: 100%;
  width: 100%;
  margin: 2rem auto;
  flex-grow: 1;

  article {
    margin: 2rem;
    font-size: 18px;
    line-height: 2;
    padding: 1rem;
    white-space: pre-line;
  }
`

export const PostTitle = styled.header`
  display: flex;
  flex-direction: column;
  align-items: left;
  justify-content: center;

  padding: 2.5rem;
  gap: 1rem;
  color: ${props => props.theme['accent-100']};

  background: #FFFFFF05;
  border: 1px solid ${props => props.theme['secondary-400']}70;
  border-bottom: 1px solid ${props => props.theme['secondary-400']}40;
  border-right: 1px solid ${props => props.theme['secondary-400']}40;
  border-radius: 3px;
  box-shadow: 0 25px 45px rgba(0, 0, 0, .1);
  backdrop-filter: blur(45px);

  small {
    color: ${props => props.theme['secondary-500']};
  }
`

export const PostTitleBackLink = styled(Link)`
  margin-right: auto;
  text-decoration: none;
  color: ${props => props.theme['accent']};

  &:hover {
    color: ${props => props.theme['primary-900']};
    background: ${props => props.theme['accent-200']};
    text-decoration: underline;
  }
`
