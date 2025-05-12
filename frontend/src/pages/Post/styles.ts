import styled from 'styled-components'

export const PostContainer = styled.div`
  max-width: 76rem;
  height: 100%;
  width: 100%;
  margin: -3rem auto;

  article {
    margin: 2rem;
    font-size: 18px;
    line-height: 2;
    padding: 1rem;
  }
`

export const PostTitle = styled.div`
  display: flex;
  flex-direction: column;
  align-items: left;
  justify-content: center;

  padding: 2.5rem;
  gap: 1rem;
  border-radius: 3px;
  background: ${props => props.theme['secondary-900']};
  border: 1px solid ${props => props.theme['secondary-800']};

  a {
    text-decoration: none;
    color: ${props => props.theme['accent']};
  }

`
