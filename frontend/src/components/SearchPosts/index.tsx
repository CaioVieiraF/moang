import { Search } from 'react-feather'
import { SearchButton, SearchInput, SearchPostsContainer } from './styles'
import { useForm } from 'react-hook-form'
import * as z from 'zod'
import { zodResolver } from '@hookform/resolvers/zod'

const SearchFormSchema = z.object({
  query: z.string(),
})

type SearchFormSchemaType = z.infer<typeof SearchFormSchema>

export function SearchPosts() {
  const { register, handleSubmit, formState: { isSubmitting } } = useForm<SearchFormSchemaType>({
    resolver: zodResolver(SearchFormSchema),
  })

  function handleSearch(data: SearchFormSchemaType) {
    console.log(data)
    // TODO implementar busca de post por título no backend
  }

  return (
    <SearchPostsContainer onSubmit={handleSubmit(handleSearch)}>
      <SearchInput type="text" placeholder="Buscar um post..." {...register('query')} disabled={isSubmitting} />
      <SearchButton disabled={isSubmitting}><Search size={20} /> Buscar</SearchButton>
    </SearchPostsContainer>
  )
}
