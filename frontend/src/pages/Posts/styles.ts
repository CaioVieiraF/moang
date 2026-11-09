import styled from 'styled-components'

export const PostsContainer = styled.main`
  max-width: 76rem;
  height: 100%;
  width: 90%;
  flex-grow: 1;

  margin: 2.5rem auto;
  border-radius: 3px;
`

export const Title = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  gap: 0.75rem;

  margin: 2rem auto;

  small {
    color: ${props => props.theme['text-300']};
  }

  @media (max-width: 600px) {
    margin: 1.5rem auto;
  }
`

export const Tabs = styled.div`
  display: flex;
  align-items: center;
  gap: 0.5rem;
`

export const TabButton = styled.button<{ $active: boolean }>`
  padding: 0.45rem 1.1rem;
  border-radius: 999px;
  border: 1px solid ${props => props.theme['secondary-400']}70;
  background: ${props => props.$active
    ? `${props.theme['accent-400']}20`
    : 'transparent'};
  color: ${props => props.$active
    ? props.theme['accent-100']
    : props.theme['secondary-300']};
  font-weight: ${props => props.$active
    ? 'bold'
    : 'normal'};
  cursor: pointer;

  &:hover {
    color: ${props => props.theme['accent-100']};
    border-color: ${props => props.theme['accent-400']}70;
  }
`

export const PostsList = styled.ul`
  list-style-type: none;
`

export const StateMessage = styled.p`
  padding: 2rem 0;
  text-align: center;
  color: ${props => props.theme['secondary-300']};

  button {
    border: 0;
    background: transparent;
    color: ${props => props.theme['accent-200']};
    font-weight: bold;
    text-decoration: underline;
    cursor: pointer;
  }
`
