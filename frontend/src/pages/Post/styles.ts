import styled from 'styled-components'

export const PostContainer = styled.div`
  max-width: 76rem;
  height: 100%;
  width: 100%;
  margin: -3rem auto;

  .titleArea {
    display: flex;
    flex-direction: column;
    align-items: left;
    justify-content: center;

    padding: 2.5rem;
    gap: 1rem;
    border-radius: 3px;
    box-shadow: 2px 2px 8px rgba(0, 0, 0, 0.7);
    background: ${props => props.theme['secondary-900']};

    a {
      text-decoration: none;
      color: ${props => props.theme['accent']};
    }
  }


  article {
    margin: 2rem;
    font-size: 18px;
    line-height: 2;
    padding: 1rem;
  }
`
