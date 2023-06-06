
//------------------------------------------------------------------------------
// This code was generated by a tool.
//
//   Tool : Bond Compiler 0.12.1.0
//   Input filename:  D:\Rust\ConnectorVertical\RustCV\src\ConnectorVertical\Bond\Microsoft.SubstrateSearch.V2.Internal.SearchRequest.bond
//   Output filename: Microsoft.SubstrateSearch.V2.Internal.SearchRequest_types.h
//
// Changes to this file may cause incorrect behavior and will be lost when
// the code is regenerated.
// <auto-generated />
//------------------------------------------------------------------------------

#pragma once

#include <bond/core/bond_version.h>

#if BOND_VERSION < 0x0902
#error This file was generated by a newer version of the Bond compiler and is incompatible with your version of the Bond library.
#endif

#if BOND_MIN_CODEGEN_VERSION > 0x0c10
#error This file was generated by an older version of the Bond compiler and is incompatible with your version of the Bond library.
#endif

#include <bond/core/config.h>
#include <bond/core/containers.h>



namespace Microsoft
{
namespace SubstrateSearch
{
namespace V2
{
namespace Internal
{
    
    struct SearchRequest
    {
        ::bond::maybe<std::string> UserCacheKey;
        
        SearchRequest()
        {
        }

        
        // Compiler generated copy ctor OK
        SearchRequest(const SearchRequest&) = default;
        
        SearchRequest(SearchRequest&&) = default;
        
        
        // Compiler generated operator= OK
        SearchRequest& operator=(const SearchRequest&) = default;
        SearchRequest& operator=(SearchRequest&&) = default;

        bool operator==(const SearchRequest& other) const
        {
            return true
                && (UserCacheKey == other.UserCacheKey);
        }

        bool operator!=(const SearchRequest& other) const
        {
            return !(*this == other);
        }

        void swap(SearchRequest& other)
        {
            using std::swap;
            swap(UserCacheKey, other.UserCacheKey);
        }

        struct Schema;

    protected:
        void InitMetadata(const char*, const char*)
        {
        }
    };

    inline void swap(::Microsoft::SubstrateSearch::V2::Internal::SearchRequest& left, ::Microsoft::SubstrateSearch::V2::Internal::SearchRequest& right)
    {
        left.swap(right);
    }
} // namespace Internal
} // namespace V2
} // namespace SubstrateSearch
} // namespace Microsoft